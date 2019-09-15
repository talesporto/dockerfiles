import os
import sys
from subprocess import check_call, run


def versions():
    return [
        ('8-stretch', '70.0.3538.110-1~deb9u1'),
        ('10-stretch', '70.0.3538.110-1~deb9u1'),
        ('10-xenial', '71.0.3578.98-0ubuntu0.16.04.1')
    ]


def tags():
    return {
        '8-stretch-chrome-70.0.3538.110': [
            'latest', '8', '8-stretch', '8-stretch-70'
        ],
        '10-stretch-chrome-70.0.3538.110': [
            '10', '10-stretch', '10-stretch-70'
        ],
        '10-xenial-chrome-71.0.3578.98': [
            '10-xenial', '10-xenial-71'
        ]
    }


def chrome_version_to_tag(chrome_version):
    return chrome_version.split('-')[0]


def build_one(folder, chrome_version):
    print(f'Building {folder} with chrome version {chrome_version}')

    check_call([
        'docker', 'build',
        '-t', f'ngeor/node-chrome:{folder}-chrome-{chrome_version_to_tag(chrome_version)}',
        '--build-arg', f'CHROME_VERSION={chrome_version}',
        '.'
    ], cwd=folder)


def build():
    print('Building')
    # for version in versions():
    #     build_one(version[0], version[1])
    # print('Tagging')
    # for tag, aliases in tags().items():
    #     for alias in aliases:
    #         print(f'Tagging {tag} as {alias}')
    #         check_call([
    #             'docker', 'tag', f'ngeor/node-chrome:{tag}', f'ngeor/node-chrome:{alias}'
    #         ])

    # experimental tag to troubleshoot Chrome 75
    check_call([
        'docker', 'build', '-t', 'ngeor/node-chrome:experimental', '.'
    ], cwd='experimental')
    check_call([
        'docker', 'build', '-t', 'ngeor/node-chrome:v75.0.3770.90', '.'
    ], cwd='v75.0.3770.90')


def login():
    username = os.environ['DOCKER_USERNAME']
    password = os.environ['DOCKER_PASSWORD']
    run(['docker', 'login', '-u', username,
         '--password-stdin'], input=password, encoding='ascii', check=True)


def logout():
    check_call(['docker', 'logout'])


def push(tag):
    print(f'Pushing {tag}')
    check_call(['docker', 'push', f'ngeor/node-chrome:{tag}'])


def deploy():
    print('Deploying')
    login()

    # for tag, aliases in tags().items():
    #     push(tag)
    #     for alias in aliases:
    #         push(alias)

    # push experimental image
    push('experimental')
    push('v75.0.3770.90')

    logout()


if __name__ == "__main__":
    build()
    if '--deploy' in sys.argv:
        deploy()
