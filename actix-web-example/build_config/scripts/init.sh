#!/bin/bash
set +x

YUM_REPO="https://repo.huaweicloud.com/repository/conf/CentOS-7-reg.repo"
PIP_HOST="repo.huaweicloud.com"
PIP_REPO="https://repo.huaweicloud.com/repository/pypi/simple"

function init_yum_repo() {
	mkdir -p /etc/yum.repos.d/tmp/
	mv /etc/yum.repos.d/CentOS* /etc/yum.repos.d/tmp/
	cp -r CentOS-7-reg.repo /etc/yum.repos.d/

	#wget --no-check-certificate -P /etc/yum.repos.d/ "${YUM_REPO}"
	
#	cat<<'EOF'>/etc/yum.repos.d/CentOS-7-reg.repo
#[base]
#name=CentOS-$releasever - Base - repo.huaweicloud.com
#baseurl=https://repo.huaweicloud.com/centos/$releasever/os/$basearch/
##mirrorlist=https://mirrorlist.centos.org/?release=$releasever&arch=$basearch&repo=os
#gpgcheck=1
#gpgkey=https://repo.huaweicloud.com/centos/RPM-GPG-KEY-CentOS-7
#EOF
#	
	
	#ignore ssl
	echo "sslverify=0" >> /etc/yum.conf
	
	ls -l /etc/yum.repos.d/
	cat /etc/yum.repos.d/CentOS-7-reg.repo
	yum clean all
	yum makecache
}

function init_pip_repo() {
    cat<<'EOF'>/etc/pip.conf
[global]
trusted-host="${PIP_HOST}"
index-url="${PIP_REPO}"
EOF

}

function install_python() {
  #yum -y install gcc zlib openssl openssl-devel zlib-devel wget
	#mkdir /py2 && mkdir /usr/local/python2.7
	#wget -O /py2/Python-2.7.14.tgz https://www.python.org/ftp/python/2.7.14/Python-2.7.14.tgz \
	#&& tar -xzf /py2/Python-2.7.14.tgz -C /py2/
	#
	#cd /py2/Python-2.7.14 && ./configure --prefix=/usr/local/python2.7/ --enable-shared \
	#&& sed -i '219,221 s/^.//' /py2/Python-2.7.14/Modules/Setup \
	#&& make -j4 && make install
	#
	#echo "/lib" >> /etc/ld.so.conf && ldconfig \
	#&& ln -s /usr/local/python2.7/lib/libpython2.7.so.1.0 /lib/libpython2.7.so.1.0 \
	#&& ln -s /usr/local/python2.7/bin/python /usr/bin/python \
	#&& rm -rf /py2
	
	unzip setuptools-39.1.0.zip
	cd setuptools-39.1.0 && python setup.py install && cd -
	
	tar -zxf pip-8.1.2.tar.gz
	cd pip-8.1.2 && python setup.py install && cd -
}

function init_env() {
	init_yum_repo
	#network tools
	#yum install -y wget
	yum install -y epel-release
	#yum install -y supervisor 
	yum install -y net-tools 
	yum install -y iproute
	
	pip install supervisor
	#yum -y install gcc zlib openssl openssl-devel zlib-devel wget
	
	python scripts/images.py || {echo "failed to run images.py"; return 1}
}
