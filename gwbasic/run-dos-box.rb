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
    @basic = nil
    @mode = nil
    @needs_stdin = false
    @program = nil
  end

  attr_reader :dosbox
  attr_reader :basic
  attr_reader :mode
  attr_reader :needs_stdin
  attr_reader :program

  def parse!
    @dosbox = parse_dosbox
    @basic = parse_basic!
    @needs_stdin = parse_needs_stdin
    @program = parse_program
  end

  private

  # Gets the location of DOSBox.exe based on the environment variable DOSBOX.
  def parse_dosbox
    ENV['DOSBOX'] || 'C:\Program Files (x86)\DOSBox-0.74\DOSBox.exe'
  end

  # Gets the location of the basic intepreter.
  # The environment variables GWBASIC and QBASIC are used.
  def parse_basic!
    exe = ENV['GWBASIC'] || ''
    if exe.empty?
      @mode = :qbasic
      exe = ENV['QBASIC'] || ''
      raise 'Please specify the location of GWBasic or QBasic' if exe.empty?
    else
      @mode = :gwbasic
    end

    raise "Basic interpreter #{exe} not found" unless File.file?(exe)

    correct_path(File.realpath(exe))
  end

  def parse_needs_stdin
    ARGV.include?('-i') || !(ENV['CONTENT_LENGTH'] || '').empty?
  end

  def parse_program
    result = if ARGV.empty?
               parse_program_from_env
             else
               ARGV[0] || ''
             end

    raise 'Please specify the program to run' if result.empty?

    raise "File #{result} not found" unless File.file?(result)

    correct_path(File.realpath(result))
  end

  def parse_program_from_env
    if ENV['BAS']
      "/basic/src/#{ENV['BAS']}"
    else
      ''
    end
  end
end

# Takes care of the path generation, relative paths, random files, etc
class PathEnv
  # Creates an instance of this class.
  # The two arguments must be full absolute paths
  # (taken care of in Arguments class).
  def initialize(basic_exe, program_bas)
    @basic_exe = basic_exe
    @program_bas = program_bas

    # the batch file needs to be placed on the common ancestor of
    # GWBASIC.EXE and PROGRAM.BAS
    #
    # the dir of the batch file will be the C:\ drive
    @batch_dir = common_ancestor(@basic_exe, @program_bas)

    @batch_file = make_unique_random_filename(@batch_dir, 'BAT')
    @stdin_file = make_unique_random_filename(@batch_dir, 'INP')
    @stdout_file = make_unique_random_filename(@batch_dir, 'OUT')
    @dosbox_log_file = make_unique_random_filename(@batch_dir, 'LOG')
  end

  attr_reader :batch_file
  attr_reader :stdin_file
  attr_reader :stdout_file
  attr_reader :dosbox_log_file

  def cleanup
    [
      @batch_file,
      @stdin_file,
      @stdout_file,
      @dosbox_log_file
    ].each do |f|
      File.delete f
    end
  end

  def program_bas_dir_from_dos
    relative_dos_path(File.dirname(@program_bas), @batch_dir)
  end

  def program_bas_filename
    File.basename(@program_bas)
  end

  def basic_exe_from_dos
    relative_dos_path(@basic_exe, @batch_dir)
  end

  def stdin_from_dos
    relative_dos_path(@stdin_file, @batch_dir)
  end

  def stdout_from_dos
    relative_dos_path(@stdout_file, @batch_dir)
  end

  private

  def make_unique_random_filename(path, ext)
    file_already_exists = true
    while file_already_exists
      result = join(path, random_filename + '.' + ext)
      file_already_exists = File.file?(result)
    end

    result
  end

  def random_filename
    # generate random filename (must be 8.3)
    o = ('A'..'Z').to_a
    (1...8).map { o[rand(o.length)] }.join
  end

  def ancestor(path)
    parent_path = File.dirname(path)
    raise 'Could not find ancestor' if path == parent_path

    parent_path
  end

  def common_ancestor(left_path, right_path)
    return left_path if left_path == right_path

    if left_path.length > right_path.length
      common_ancestor ancestor(left_path), right_path
    else
      common_ancestor left_path, ancestor(right_path)
    end
  end

  def relative_dos_path(path, ancestor_dir)
    result = ''
    while path != ancestor_dir
      result = '\\' + result unless result.empty?

      result = File.basename(path) + result
      path = ancestor(path)
    end

    result = '\\' + result unless result.start_with?('\\')

    'C:' + result
  end
end

# Main class, runs DOSBox with GWBasic, manages temp files.
class Main
  def initialize(args)
    @args = args
    @path_env = PathEnv.new(@args.basic, @args.program)
  end

  def cleanup
    @path_env.cleanup
  end

  def create_stdin
    # touch stdin.txt
    File.open(@path_env.stdin_file, 'wb') do |f|
      if @args.needs_stdin
        while (s = STDIN.gets)
          f.print "#{s.chomp}\r\n"
        end
      end
    end
  end

  def create_batch_file
    arg = if @args.mode == :gwbasic
            [
              @path_env.basic_exe_from_dos,
              @path_env.program_bas_filename,
              "<#{@path_env.stdin_from_dos}",
              ">#{@path_env.stdout_from_dos}\r\n"
            ]
          else
            [
              @path_env.basic_exe_from_dos,
              '/RUN',
              @path_env.program_bas_filename,
              "<#{@path_env.stdin_from_dos}",
              ">#{@path_env.stdout_from_dos}\r\n"
            ]
          end

    # touch run.bat
    File.open(@path_env.batch_file, 'wb') do |f|
      copy_env f
      f.print "SET STDIN=#{@path_env.stdin_from_dos}\r\n"

      # CD into the folder of where the BAS file lives, so that by default it
      # reads/writes files in its own folder
      f.print "CD #{@path_env.program_bas_dir_from_dos}\r\n"
      f.print arg.join(' ')
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
      @args.dosbox, @path_env.batch_file, '-exit', '-noautoexec',
      %i[out err] => @path_env.dosbox_log_file
    )
  end

  def print_stdout
    # Output stdout
    File.open(@path_env.stdout_file, 'r') do |f|
      while (s = f.gets)
        puts s.chomp
      end
    end
  end

  private

  def copy_env(file)
    # TODO: support ENV for QBasic too
    if @args.mode == :gwbasic
      ENV.each do |k, v|
        if k != 'PATH' && k =~ /^[A-Z][A-Z_]+$/ && !v.empty? && !v.include?(' ')
          file.print "SET #{k}=#{v}\r\n"
        end
      end
    end
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
