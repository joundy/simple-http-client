## Simple HTTP Client

This project is not intended for use in a production environment; it is a fun project for learning HTTP requests using plain TCP streams.

example output:
```
[src/main.rs:190] header_response = {
    "content-length": "519",
    "etag": "W/\"207-QowjjkZS3dPvv4L6zPF2KPB7cKk\"",
    "content-type": "application/json; charset=utf-8",
    "vary": "Accept-Encoding",
    "x-download-options": "noopen",
    "x-dns-prefetch-control": "off",
    "x-xss-protection": "1; mode=block",
    "access-control-allow-origin": "*",
    "x-content-type-options": "nosniff",
    "date": "Fri, 13 Oct 2023 18:03:27 GMT",
    "x-frame-options": "SAMEORIGIN",
    "x-ratelimit-remaining": "118",
    "x-ratelimit-reset": "1697220235",
    "connection": "close",
    "server": "railway",
    "x-ratelimit-limit": "120",
    "strict-transport-security": "max-age=15552000; includeSubDomains",
}
[src/main.rs:193] body_response = "{\"id\":1,\"title\":\"iPhone 9\",\"description\":\"An apple mobile which is nothing like apple\",\"price\":549,\"discountPer
centage\":12.96,\"rating\":4.69,\"stock\":94,\"brand\":\"Apple\",\"category\":\"smartphones\",\"thumbnail\":\"https://i.dummyjson.com/data/products/1/thumbnail
.jpg\",\"images\":[\"https://i.dummyjson.com/data/products/1/1.jpg\",\"https://i.dummyjson.com/data/products/1/2.jpg\",\"https://i.dummyjson.com/data/products/
1/3.jpg\",\"https://i.dummyjson.com/data/products/1/4.jpg\",\"https://i.dummyjson.com/data/products/1/thumbnail.jpg\"]}"
```
