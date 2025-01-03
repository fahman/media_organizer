# Media Organizer

This is a repository to house the code related to organizing media into one folder with a date-centric folder structure.

View the crates related read me files with these links.

- [Media Organizer](./media_organizer/readme.md)
- [Media Info](./media_info/README.md)
- [File Metadata](./fs_metadata/readme.md)

## Examples
Copy (```-c```) from source (```-s D:\iPhone.Photos```) into destination relative to current directory (```-d iPhone.Photos1```)
```
..../media_organizer.exe -s D:\iPhone.Photos -d iPhone.Photos1 -c
```

## TODO

1. Github build and deploy to crate.
2. AI classification using Rust Candle.
 - https://huggingface.co/spaces/radames/Candle-BLIP-Image-Captioning
 - https://huggingface.github.io/candle/guide/installation.html
