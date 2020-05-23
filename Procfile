APP="finding-nemo-jj" && \
cargo new --bin $APP      && \
cd $APP                   && \
git init                  && \
heroku create $APP --buildpack https://github.com/Hoverbear/heroku-buildpack-rust && \
echo "web: target/release/$APP"