#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';
use Cwd;
use POSIX qw(strftime);
use LWP::UserAgent;
use JSON;

# Login to Docker CLI
sub docker_login {
  my ($username, $password) = @_;
  if (!$username || !$password) {
    die "Missing Docker credentials";
  }

  system("docker login -u $username -p $password") == 0 or die "could not login";
}

# Logout of Docker CLI
sub docker_logout {
  system("docker logout") == 0 or die "could not logout";
}


sub hub_login {
  my %args = (
    api_version => 2,
    @_
  );

  my $api_version = $args{api_version};
  my $path = "users/login";
  my $username = $args{username};
  my $password = $args{password};

  my $req = HTTP::Request->new(POST => "https://hub.docker.com/v${api_version}/${path}");
  $req->content_type("application/json");
  $req->content(encode_json { username => $username, password => $password });

  my $ua = LWP::UserAgent->new;
  my $res = $ua->request($req);
  my $jwt_token;
  if ($res->is_success) {
    my $body = decode_json $res->decoded_content;
    return $body->{token};
  } else {
    die "Could not login: $res->status_line";
  }
}

sub hub_set_description {
  my %args = (
    api_version => 2,
    @_
  );

  my $api_version = $args{api_version};
  my $jwt_token = $args{jwt_token};
  my $username = $args{username};
  my $name = $args{name};
  my $description = $args{description};
  my $full_description = $args{full_description};
  my $path = "repositories/$username/$name/";

  my $req = HTTP::Request->new(PATCH => "https://hub.docker.com/v${api_version}/${path}");
  $req->content_type("application/json");
  $req->header(Authorization => "JWT ${jwt_token}");
  $req->content(encode_json { description => $description, full_description => $full_description });

  my $ua = LWP::UserAgent->new;
  my $res = $ua->request($req);
  if (!$res->is_success) {
    die "Could not set description: $res->status_line";
  }
}

# Generates the full description for a Docker image.
# This is run while the current directory is the Docker image's folder,
# so we check if there is a README.md and if one exists, it is appended to
# the description.
sub generate_full_description {
  my ($folder, $username, $description) = @_;
  my $full_description = <<HERE;
$description

[Dockerfile](https://github.com/$username/dockerfiles/blob/master/$folder/Dockerfile)

HERE

  if (-r 'README.md') {
    open my $fh, '<', 'README.md';
    while (my $line = <$fh>) {
      $full_description .= $line;
    }
    close $fh;
  }

  return $full_description;
}

# Process all subfolders, build and push images
sub process {
  my ($username, $password) = @_;

  # when the name is missing, use the same as the folder name
  my @data = (
    {
      folder      => "awscli-alpine",
      name        => "awscli",
      description => "Python Alpine with AWS CLI"
    },
    {
      folder      => "awscli-terraform-alpine",
      name        => "awscli-terraform",
      description => "Python Alpine with AWS CLI and terraform"
    },
    {
      folder      => "az-helm-kubectl-terraform",
      description => "Azure CLI plus kubectl, helm, and terraform"
    },
    {
      folder      => "helm-kubectl-terraform",
      description => "kubectl, helm, and terraform"
    },
    {
      folder      => "jdk-helm-kubectl-terraform",
      description => "JDK, maven, ant, gradle, kubectl, helm, and terraform"
    },
    {
      folder      => "maven-awscli",
      description => "JDK, maven, python, and AWS CLI"
    },
    {
      folder      => "node-chrome",
      description => "nodeJS 10 and Chrome headless"
    },
    {
      folder      => "node-firefox",
      description => "nodeJS 10 and Firefox headless"
    },
    {
      folder      => "python-helm-kubectl-terraform",
      description => "Python, kubectl, helm, and terraform"
    },
    {
      folder      => "ruby-helm-kubectl-terraform",
      description => "Ruby, kubectl, helm, and terraform"
    },
    {
      folder      => "swagger-to-diagram",
      description => "Converts Swagger definitions to PlantUML diagrams"
    },
    {
      folder      => "vsftpd",
      description => "FTPS image based on vsftpd"
    }
  );

  my %folder_to_data = map { $_->{folder} => $_ } @data;

  my @changed_files = split /[\r\n]/, qx(git diff --name-only HEAD^);
  my $cwd = getcwd();
  my $date = strftime "%Y%m%d-%H%M%S", gmtime;
  my $is_master = $ENV{TRAVIS_BRANCH} eq "master" && $ENV{TRAVIS_PULL_REQUEST} eq "false";
  my $jwt_token = hub_login(username => $username, password => $password);

  foreach my $folder (keys(%folder_to_data)) {
    my $data = $folder_to_data{$folder};
    my $name = $data->{name} || $folder;
    chdir "$cwd/$folder";

    if (grep { /$folder/ } @changed_files) {
      my $fqn = "$username/$name:$date";
      say "Building folder $folder as $fqn";
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

    # update description
    hub_set_description(
      jwt_token        => $jwt_token,
      description      => $data->{description},
      full_description => generate_full_description($folder, $username, $data->{description}),
      username         => $username,
      name             => $name
    );
  }
}

my $username = $ENV{DOCKER_USERNAME};
my $password = $ENV{DOCKER_PASSWORD};
docker_login($username, $password);
process($username, $password);
docker_logout();
