#!/usr/bin/env perl
use strict;
use warnings;
use v5.26;
use POSIX qw(strftime);
use LWP::UserAgent;
use JSON;

sub main {
    docker_login();
    my $hub_jwt_token = hub_login();
    process($hub_jwt_token);
    docker_logout();
}

# Login to Docker CLI
sub docker_login {
    my $username = docker_username();
    my $password = docker_password();
    system("docker login -u $username -p $password") == 0 or die "could not login";
}

sub docker_username {
    return $ENV{DOCKER_USERNAME};
}

sub docker_password {
    return $ENV{DOCKER_PASSWORD};
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
    my $path        = "users/login";
    my $username    = docker_username();
    my $password    = docker_password();

    my $req = HTTP::Request->new(POST => "https://hub.docker.com/v${api_version}/${path}");
    $req->content_type("application/json");
    $req->content(encode_json { username => $username, password => $password });

    my $ua = LWP::UserAgent->new;
    my $res = $ua->request($req);
    if ($res->is_success) {
        my $body = decode_json $res->decoded_content;
        return $body->{token};
    } else {
        die "Could not login: ", $res->status_line;
    }
}

# Process all subfolders, build and push images
sub process {
    my ($hub_jwt_token) = @_;

    # when the name is missing, use the same as the folder name
    my @folders = (
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
        },
        {
            folder => "gwbasic",
            images => [
                {
                    file        => "Dockerfile.standalone",
                    description => "Launch GW-Basic with DOSBox",
                    name        => "gwbasic",
                },
                {
                    file        => "Dockerfile.httpd",
                    description => "Apache HTTPD with GW-Basic as cgi-bin",
                    name        => "gwbasic-httpd",
                }
            ]
        },
    );

    my @changed_files = split /[\r\n]/, qx(git diff --name-only HEAD^);
    my $date          = strftime "%Y%m%d-%H%M%S", gmtime;

    foreach my $data (@folders) {
        process_folder(
            data          => $data,
            changed_files => \@changed_files,
            hub_jwt_token => $hub_jwt_token,
            date          => $date,
        );
    }
}

sub process_folder {
    my %args = @_;
    my $data          = $args{data};
    my $changed_files = $args{changed_files};
    my $hub_jwt_token = $args{hub_jwt_token};
    my $date          = $args{date};

    my @images = @{get_images($data)};
    for my $image (@images) {
        process_image(
            data          => $data,
            changed_files => $changed_files,
            hub_jwt_token => $hub_jwt_token,
            date          => $date,
            image         => $image,
        );
    }
}

sub get_images {
    my ($data) = @_;
    my $images = $data->{images};
    if ($images) {
        return $images;
    }

    return [ $data ];
}

sub process_image {
    my %args = @_;
    # unpack arguments
    my $data          = $args{data};
    my $changed_files = $args{changed_files};
    my $hub_jwt_token = $args{hub_jwt_token};
    my $date          = $args{date};
    my $image         = $args{image};
    # calculate values
    my $folder      = $data->{folder};
    my $has_changes = grep { /$folder/ } @{$changed_files};
    my $tag         = get_tag($folder);

    if (!$has_changes && !$tag) {
        say "Not building $folder because it has not changed and it is not tagged";
        return;
    }

    my $name       = $image->{name} || $folder;
    my $dockerfile = $image->{file} || 'Dockerfile';
    my $username   = docker_username();
    my $fqn        = "$username/$name:$date";
    say "Building folder $folder as $fqn";
    system("docker build -f $folder/$dockerfile -t $fqn $folder") == 0 or die "could not build image $name";
    if ($tag) {
        docker_tag_and_push($fqn, "$username/$name:$tag");
    }
    if (is_master()) {
        docker_tag_and_push($fqn, "$username/$name:latest");
    }

    # update description
    hub_set_description(
        hub_jwt_token    => $hub_jwt_token,
        description      => $image->{description},
        full_description => generate_full_description($folder, $image->{description}, $dockerfile),
        name             => $name
    );
}

sub get_tag {
    my ($folder) = @_;
    my $travis_tag = $ENV{TRAVIS_TAG};
    if ($travis_tag =~ /^(v[0-9\.]+)-$folder$/) {
        return $1;
    }

    return "";
}

sub is_master {
    return $ENV{TRAVIS_BRANCH} eq "master" && $ENV{TRAVIS_PULL_REQUEST} eq "false";
}

sub docker_tag_and_push {
    my ($fqn, $fqn_new) = @_;
    system("docker tag $fqn $fqn_new") == 0 or die "could not tag $fqn as $fqn_new";
    system("docker push $fqn_new") == 0 or die "could not push $fqn_new";
}

sub hub_set_description {
    my %args = (
        api_version => 2,
        @_
    );

    my $api_version      = $args{api_version};
    my $name             = $args{name};
    my $hub_jwt_token    = $args{hub_jwt_token};
    my $description      = $args{description};
    my $full_description = $args{full_description};
    my $username         = docker_username();
    my $path             = "repositories/$username/$name/";

    my $req = HTTP::Request->new(PATCH => "https://hub.docker.com/v${api_version}/${path}");
    $req->content_type("application/json");
    $req->header(Authorization => "JWT ${hub_jwt_token}");
    $req->content(encode_json { description => $description, full_description => $full_description });

    my $ua = LWP::UserAgent->new;
    my $res = $ua->request($req);
    if (!$res->is_success && res->code != 404) {
        die "Could not set description: ", $res->status_line;
    }
}

# Generates the full description for a Docker image.
# This is run while the current directory is the Docker image's folder,
# so we check if there is a README.md and if one exists, it is appended to
# the description.
sub generate_full_description {
    my ($folder, $description, $file) = @_;
    my $username = docker_username();
    my $full_description = <<HERE;
$description

[Dockerfile](https://github.com/$username/dockerfiles/blob/master/$folder/$file)

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

main();
