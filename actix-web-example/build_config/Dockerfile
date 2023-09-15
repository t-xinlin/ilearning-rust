FROM centos:7

ENV LANG="en_US.UTF-8"
ENV LANGUAGE="en_US.UTF-8"
ENV LC_ALL="en_US.UTF-8"
ENV INSTALL_DIR="/opt/service/actix-web"

COPY . /tmp/service-build
WORKDIR /tmp/service-build

RUN bash -c ". scripts/init.sh; init_env" \
    && groupadd -g 9999 svcgroup \
    && useradd -u 9999 -g 9999 service \
    && chown -R 9999:9999 /home/service \
	&& mkdir -p $INSTALL_DIR/conf \
	&& mkdir -p $INSTALL_DIR/bin/conf \
	&& mkdir -p $INSTALL_DIR/shell \
    && chmod -R 700 $INSTALL_DIR/ \
	&& cp -r conf/* $INSTALL_DIR/bin/conf \
    && cp start.sh $INSTALL_DIR/bin \
	&& cp actix-web-example $INSTALL_DIR/bin \
	&& cp actix-web-example.db $INSTALL_DIR/bin \
    && rm -rf /tmp/service-build

EXPOSE 8081
CMD ["/opt/service/supervisord", "-c", "/opt/service/supervisord.conf"]
