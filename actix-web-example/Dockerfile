FROM alpine

ENV LANG="en_US.UTF-8"
ENV LANGUAGE="en_US.UTF-8"
ENV LC_ALL="en_US.UTF-8"
ENV INSTALL_DIR="/opt/service/actix-web"

COPY . /tmp/service-build
WORKDIR /tmp/service-build

RUN bash -c ".script/build.sh; init" \
    && chown -R 1001:1001 /home/service \
    && chmod 7000 /opt/service \
    && cp start.sh $INSTALL_DIR/shell \
    && cp install.sh $INSTALL_DIR/shell \
    && sh -x $INSTALL_DIR/shell/install.sh init \
    && rm -rf /tmp/service-build

EXPOSE 8081
CMD["/opt/service/supervisord", "-c", "/opt/service/supervisord.conf"]
