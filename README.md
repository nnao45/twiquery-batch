# twiquery-batch

```
$ cat <<EOF > ./config.yaml                                                                                                                                        CONSUMER_KEY: <twitter consumer key>
CONSUMER_SECRET: <twitter consumer secret>
ACCESS_TOKEN: <twitter access token>
ACCESS_TOKEN_SECRET: <twitter access token secret>
TRACK: <twitter search keyword>
SLACK_URL: <post slack webhook url>
EOF
$ cargo run --release
```