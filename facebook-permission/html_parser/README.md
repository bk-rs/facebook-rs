## Dev

```
curl 'https://developers.facebook.com/docs/permissions/reference' \
  -H 'authority: developers.facebook.com' \
  -H 'pragma: no-cache' \
  -H 'cache-control: no-cache' \
  -H 'viewport-width: 1368' \
  -H 'sec-ch-ua: "Chromium";v="91", " Not;A Brand";v="99"' \
  -H 'sec-ch-ua-mobile: ?0' \
  -H 'upgrade-insecure-requests: 1' \
  -H 'user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.114 Safari/537.36' \
  -H 'accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9' \
  -H 'sec-fetch-site: same-origin' \
  -H 'sec-fetch-mode: navigate' \
  -H 'sec-fetch-user: ?1' \
  -H 'sec-fetch-dest: document' \
  -H 'referer: https://developers.facebook.com/docs/permissions/reference' \
  -H 'accept-language: en-US,en;q=0.9,zh-CN;q=0.8,zh;q=0.7' \
  -H 'cookie: xxx' \
  -o tests/developers_docs_permissions_reference.html
```
