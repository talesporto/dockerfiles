#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';
use Cwd;
use POSIX qw(strftime);

sub docker_login {
  my $username = shift;
  my $password = $ENV{DOCKER_PASSWORD};
  if (!$username || !$password) {
    die "Missing Docker credentials";
  }

  system("docker login -u $username -p $password") == 0 or die "could not login";
}

sub docker_logout {
  system("docker logout") == 0 or die "could not logout";
}

sub process {
  # empty string as value means use the same as the folder name
  my $username = shift;
  my %folder_to_name = (
    "awscli-alpine"                 => "awscli",
    "awscli-terraform-alpine"       => "awscli-terraform",
    "az-helm-kubectl-terraform"     => "",
    "helm-kubectl-terraform"        => "",
    "jdk-helm-kubectl-terraform"    => "",
    "maven-awscli"                  => "",
    "node-chrome"                   => "",
    "node-firefox"                  => "",
    "python-helm-kubectl-terraform" => "",
    "ruby-helm-kubectl-terraform"   => "",
    "swagger-to-diagram"            => "",
  );

  my @changed_files = split /[\r\n]/, qx(git diff --name-only HEAD^);
  my $cwd = getcwd();
  my $date = strftime "%Y%m%d-%H%M%S", gmtime;
  my $is_master = $ENV{TRAVIS_BRANCH} eq "master" && $ENV{TRAVIS_PULL_REQUEST} eq "false";

  foreach my $folder (keys(%folder_to_name)) {
    if (grep { /$folder/ } @changed_files) {
      my $name = $folder_to_name{$folder} || $folder;
      my $fqn = "$username/$name:$date";
      say "Building folder $folder as $fqn";
      chdir "$cwd/$folder";
      system("docker build -t $fqn .") == 0 or die "could not build image $name";
      system("docker push $fqn") == 0 or die "could not push $fqn";
      if ($is_master) {
        # tag latest
        my $fqn_latest = "$username/$name:latest";
        system("docker tag $fqn $fqn_latest") == 0 or die "could not tag $fqn as $fqn_latest";
        system("docker push $fqn_latest") == 0 or die "could not push $fqn_latest";
      }
    } else {
      say "Not building $folder because it has not changed";
    }
  }
}

my $username = $ENV{DOCKER_USERNAME};
docker_login($username);
process($username);
docker_logout();
