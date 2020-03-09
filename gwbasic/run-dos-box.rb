#!/usr/bin/env ruby
# frozen_string_literal: true

# On Windows (cmd prompt):
#
# ruby run-dos-box.rb HELLO.BAS
# type Dockerfile.standalone | ruby run-dos-box.rb ECHO.BAS -i
#
# On Windows (Git Bash):
#
# ./run-dos-box.rb HELLO.BAS
# cat Dockerfile.standalone | ./run-dos-box.rb ECHO.BAS -i
#
# Docker:
#
# docker build -t gwbasic -f Dockerfile.standalone .
# docker run -v $PWD:/basic:ro --rm gwbasic HELLO.BAS
# cat Dockerfile.standalone | docker run -i -v $PWD:/basic:ro --rm gwbasic ECHO.BAS -i
#
# Docker (HTTP)
#
# docker build -t gwbasic-httpd -f Dockerfile.httpd .
# docker run --rm --name gwbasic-httpd -p 8080:80 -d -v $PWD/rest:/basic:ro gwbasic-httpd
# docker stop gwbasic-httpd

def main
  args = Arguments.new
  args.parse!
  main = Main.new(args)
  main.copy_program
  main.create_stdin
  main.create_batch_file
  main.run_dosbox
  main.print_stdout
  main.cleanup
end

# Parses arguments from command line and environment variables.
class Arguments
  def initialize
    @dosbox = nil
    @gwbasic = nil
    @qbasic = nil
    @needs_stdin = false
    @program = nil
  end

  attr_reader :dosbox
  attr_reader :gwbasic
  attr_reader :qbasic
  attr_reader :needs_stdin
  attr_reader :program

  def parse!
    @dosbox = parse_dosbox
    @gwbasic = parse_gwbasic
    @qbasic = parse_qbasic
    @needs_stdin = parse_needs_stdin
    parse_program
  end

  private

  def parse_dosbox
    ENV['DOSBOX'] || 'C:\Program Files (x86)\DOSBox-0.74\DOSBox.exe'
  end

  def parse_gwbasic
    ENV['GWBASIC'] || ''
  end

  def parse_qbasic
    ENV['QBASIC'] || ''
  end

  def parse_needs_stdin
    ARGV.include?('-i') || !(ENV['CONTENT_LENGTH'] || '').empty?
  end

  def parse_program
    if ARGV.size <= 0
      warn 'Please specify the BAS file to run'
      exit!
    end

    @program = ARGV[0]
    return if File.exist?(program)

    warn "File #{program} not found"
    exit!
  end
end

# Main class, runs DOSBox with GWBasic, manages temp files.
class Main
  def initialize(args)
    @args = args
    if !args.gwbasic.empty?
      @mode = :gwbasic
      @basic_dir = File.dirname(args.gwbasic)
      @basic_exe = File.basename(args.gwbasic)
    elsif !args.qbasic.empty?
      @mode = :qbasic
      @basic_dir = File.dirname(args.qbasic)
      @basic_exe = File.basename(args.qbasic)
    else
      warn 'Please specify either GWBASIC or QBASIC env variable'
      exit!
    end

    @tmp_filename = random_filename

    # TODO: support STDIN.TXT with different filename
    # TODO: lockfile (which means it needs to live inside the data folder)
    # TODO: use glorious CRLF for BAS files (drop the shebang support)

    # the batch file needs to live at the same folder with GWBASIC.EXE,
    # so that DOSBOX mounts it as C:\
    @batch = "#{@tmp_filename}.BAT"
    @batch_full = join(@basic_dir, @batch)
    @stdin = 'STDIN.TXT' # TODO: this needs to be varying too
    @stdin_full = join(@basic_dir, @stdin)
    @stdout = "#{@tmp_filename}.OUT"
    @stdout_full = join(@basic_dir, @stdout)
    @program_copy = "#{@tmp_filename}.BAS"
    @program_copy_full = join(@basic_dir, @program_copy)
    @dosbox_log_full = join(@basic_dir, "#{@tmp_filename}.log")
  end

  def cleanup
    [
      @stdin_full,
      @batch_full,
      @program_copy_full,
      @stdout_full,
      @dosbox_log_full
    ].each do |f|
      File.delete f
    end
  end

  def copy_program
    # copy program to temporary location
    File.open(@args.program, 'r') do |f_in|
      File.open(@program_copy_full, 'wb') do |f_out|
        while (s = f_in.gets)
          # ensure CRLF, strip shebang
          f_out.print "#{s.chomp}\r\n" unless s =~ %r{^#!/}
        end
      end
    end
  end

  def create_stdin
    # touch stdin.txt
    File.open(@stdin_full, 'wb') do |f|
      if @args.needs_stdin
        while (s = STDIN.gets)
          f.print "#{s.chomp}\r\n"
        end
      end
    end
  end

  def create_batch_file
    # touch run.bat
    File.open(@batch_full, 'wb') do |f|
      if @mode == :gwbasic
        ENV.each do |k, v|
          if k != 'PATH' && k =~ /^[A-Z][A-Z_]+$/ && !v.empty? && !v.include?(' ')
            f.print "SET #{k}=#{v}\r\n"
          end
        end
      end

      if @mode == :gwbasic
        f.print "#{@basic_exe} #{@program_copy} <#{@stdin} >#{@stdout}\r\n"
      else
        f.print "#{@basic_exe} /RUN #{@program_copy} <#{@stdin} >#{@stdout}\r\n"
      end
    end
  end

  def run_dosbox
    env = {
      'SDL_VIDEODRIVER' => 'dummy',
      'TERM' => 'xterm'
    }

    # create process
    system(
      env,
      @args.dosbox, @batch_full, '-exit', '-noautoexec',
      %i[out err] => @dosbox_log_full
    )
  end

  def print_stdout
    # Output stdout
    File.open(@stdout_full, 'r') do |f|
      while (s = f.gets)
        puts s.chomp
      end
    end
  end

  private

  def random_filename
    # generate random filename (must be 8.3)
    o = ('A'..'Z').to_a
    (1...8).map { o[rand(o.length)] }.join
  end
end

def windows?
  (/cygwin|mswin|mingw/ =~ RUBY_PLATFORM) != nil
end

def correct_path(path)
  if windows?
    path.gsub('/', '\\')
  else
    path
  end
end

def join(left_path, right_path)
  correct_path(File.join(left_path, right_path))
end

main
