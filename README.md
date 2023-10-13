## Simple HTTP Client

This project is not intended for use in a production environment; it is a fun project for learning HTTP requests using plain TCP streams.

example output:
```
[src/main.rs:190] header_response = {
    "Last-Modified": "Thu, 17 Oct 2019 07:18:26 GMT",
    "Cache-Control": "max-age=604800",
    "Age": "185568",
    "Vary": "Accept-Encoding",
    "Etag": "\"3147526947\"",
    "Server": "ECS (laa/7B10)",
    "Expires": "Fri, 20 Oct 2023 18:01:20 GMT",
    "Accept-Ranges": "bytes",
    "Content-Length": "1256",
    "Connection": "close",
    "Content-Type": "text/html; charset=UTF-8",
    "Date": "Fri, 13 Oct 2023 18:01:20 GMT",
    "X-Cache": "HIT",
}
[src/main.rs:193] body_response = "<!doctype html>\n<html>\n<head>\n    <title>Example Domain</title>\n\n    <meta charset=\"utf-8\" />\n    <meta http-equiv=\
"Content-type\" content=\"text/html; charset=utf-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n    <style type=\"text
/css\">\n    body {\n        background-color: #f0f0f2;\n        margin: 0;\n        padding: 0;\n        font-family: -apple-system, system-ui, BlinkMacSystem
Font, \"Segoe UI\", \"Open Sans\", \"Helvetica Neue\", Helvetica, Arial, sans-serif;\n        \n    }\n    div {\n        width: 600px;\n        margin: 5em au
to;\n        padding: 2em;\n        background-color: #fdfdff;\n        border-radius: 0.5em;\n        box-shadow: 2px 3px 7px 2px rgba(0,0,0,0.02);\n    }\n  
  a:link, a:visited {\n        color: #38488f;\n        text-decoration: none;\n    }\n    @media (max-width: 700px) {\n        div {\n            margin: 0 au
to;\n            width: auto;\n        }\n    }\n    </style>    \n</head>\n\n<body>\n<div>\n    <h1>Example Domain</h1>\n    <p>This domain is for use in illu
strative examples in documents. You may use this\n    domain in literature without prior coordination or asking for permission.</p>\n    <p><a href=\"https://w
ww.iana.org/domains/example\">More information...</a></p>\n</div>\n</body>\n</html>\n"
```
