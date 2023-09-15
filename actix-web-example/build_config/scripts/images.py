import shutil
import subprocess
import logging
import sys
import os

logging.basicConfig(level=logging.INFO, stream=sys.stderr,
                    format='[%(asctime)s] [%(module)s/%(funcName)s] [%(levelname)s] %(message)s')
LOGGER = logging.getLogger(__name__)

TO_INSTALL = ['epel-release', 'libyaml', 'tree', 'lsof', 'bind-utils', 'net-tools', 'iproute', 'wget', 'supervisor']

def exec_cmd(cmd):
    LOGGER.info("exec cmd: %s" % cmd)
    proc = subprocess.Popen(
        cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True, preexec_fn=os.setsid)
    stdout, stderr = proc.communicate()
    recode = proc.poll()

    return recode, stdout, stderr

def init_yum():
    for pkg in TO_INSTALL:
        rc, _, err = exec_cmd('yum install -y ' + pkg)
        if rc != 0:
            raise Exception('failed to install {0}, Exception: {1}'.format(pkg, err))

PIP_REPO_CONTENT = '''[global]
trusted-host=repo.huaweicloud.com
index-url=https://repo.huaweicloud.com/repository/pypi/simple
'''

def init_pip(backup=False):
    pip_conf = "/etc/pip.conf"
    if os.path.isfile(pip_conf):
        if backup:
            shutil.move(pip_conf, os.path.basename(pip_conf) + ".-tmp-bk")
        else:
            os.remove(pip_conf)

    with open("/etc/pip.conf", "w") as f:
        f.write(PIP_REPO_CONTENT)

if __name__ == '__main__':
    try:
        init_yum()
    except Exception as e:
        LOGGER.exception("failed to init yum repo")
        sys.exit(1)
