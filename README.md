# photo-importer

A simple photo import tool.

You can copy photos into folders by date.

This tool follows the folder naming rule of Luminar3.

## Folder name and structure

```
2022
└── 06
    └── 19
        ├── P1110189.JPG
        ├── P1110190.JPG
        ├── P1110191.JPG
        └── P1110192.JPG

```

## Usage

```
$ photo-importer [CAMERA FOLDER] [OPTIONAL: DESTINATION FOLDER]
```

### Example

**/media/takumi/LUMIX/DCIM ---> ~/Pictures**

The default destination folder is `~/Pictures`.

```
$ photo-importer /media/takumi/LUMIX/DCIM
```

**/media/takumi/LUMIX/DCIM ---> /media/takumi/data_drive/photos**

You can specify the default destination folder as the second argment.

```
$ photo-importer /media/takumi/LUMIX/DCIM /media/takumi/data_drive/photos
```
