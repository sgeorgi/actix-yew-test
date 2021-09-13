FROM alpine:3.13.6
ARG APP=/usr/src/app
EXPOSE 9000

ENV TZ=Etc/UTC \
    APP_USER=appuser
RUN addgroup -S $APP_USER && adduser -S $APP_USER -G $APP_USER

COPY ./target/release/bundle/* ${APP}/
RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["/usr/src/app/server"]
