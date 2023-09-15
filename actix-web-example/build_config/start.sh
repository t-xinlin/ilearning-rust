#!/bin/bash
set -e
cur_path=`pwd`
local_ip=""
f_path=${cur_path}/test.txt
export IGNORE_CASE=true

function ipAddress() {
    # 获取IP命令
    ipaddr=`ifconfig -a|grep eth0 -A 3|grep inet|grep -v 127.0.0.1|grep -v inet6|awk '{print $2}'|tr -d "addr:"`
    array=(`echo $ipaddr | tr '\n' ' '` ) # IP地址分割，区分是否多网卡
    num=${#array[@]} #获取数组元素的个数

    # 选择安装的IP地址
    if [ $num -eq 1 ]; then
      #echo "*单网卡"
      local_ip=${array[*]}
    elif [ $num -gt 1 ];then
      echo -e "\033[035m******************************\033[0m"
      echo -e "\033[036m* 请选择安装的IP地址 \033[0m"
      for ((i=0; i<$num; i++))
      do
        echo -e "\033[032m* $[$i+1] : ${array[$i]} \033[0m"
      done
      echo -e "\033[035m******************************\033[0m"

      #选择需要安装的服务类型
      input=""
      while :
      do
        #read -r -p "*请选择安装的IP地址(序号): " input
        #chose=$[$input-1]
		chose=0
        flag=0
        for ((i=0; i<$num; i++))
        do
          if [ $chose -eq $i ]; then
            flag=1
            break
          fi
        done
        if [ $flag -eq 0 ]; then
          echo "*请输入有效的数字:"
          continue
        fi
        local_ip=${array[$chose]}
        break
      done
    else
      echo -e "\033[31m*未设置网卡IP，请检查服务器环境！ \033[0m"
      exit 1
    fi
}
ipAddress
echo $local_ip

sed -i "0,/address = \"{{addr}}\"/s/address = \"{{addr}}\"/address = \"${local_ip}\"/g" ${cur_path}/conf/app.toml
local_host=127.0.0.1
sed -i "0,/address = \"{{addr}}\"/s/address = \"{{addr}}\"/address = \"${local_host}\"/" ${cur_path}/conf/app.toml

#sed -i "s/port = \"{{port}}\"/port = ${port}/g" conf/app.toml
# 删除不设置的
sed -i "/address = \"{{addr}}\"/d" ${cur_path}/conf/app.toml

./actix-web-example abc $f_path
