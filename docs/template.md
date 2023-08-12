# Template Format

This document describes the format of the template folder.

## Template folder

The template folder is a preconfigured directory that contains the template files their metadata. The template folder subdirectories which each represent a single template.

```sh
templates/                      # templates folder
├─ <name_of_template>/          # template folder
│  ├─ template.document.html    # template document file (to be used in exports)
│  ├─ template.body.html        # template layout file (to be used everywhere)
│  ├─ template.css              # template style file
│  ├─ template.json             # template metadata file
```

## Template metadata

## Template files

Template files are simply just HTML files, but can contain special sections where data, such as body HTML, frontmatter data, CSS data, may be filled in.

### Template document file
This file contains the document template, including the `<html>`, `<head>`, and `<body>` tags. This file is used for exports.

For example:
```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <title>{{ fm.title }}</title>
    <style>
      {{ css }}
    </style>
  </head>
  <body>
    <h1>{{ fm.title }}</h1>
    {{ body }}
  </body>
</html>
```

### Template body file
This file contains the body template, which is placed the `<body>` tag in the preview, and is accessible from the `{{ body }}` variable in the document template. This file is used for the content layout of the document.

For example:
```html
<style>{{ css }}</style>
<h1>{{ fm.title }}</h1>
{{ body }}
```

