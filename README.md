# trash-rs
A trash-cli copy, but this time in Rust.

### TODO

- [x] Deletion
  - [x] Relative paths
  - [x] * functionality
  - [ ] External file to save original file locations
  - [ ] Non-empty directories (recursive solution coming up!)
  - [ ] No overwrites
    - [ ] Overwrite allowed flag
- [x] List files
  - [ ] Include in Metadata:
    - [ ] Original directory
    - [x] Date deleted
- [ ] Empty trash
  - [ ] * functionality
  - [ ] -y flag to skip confirmation step
- [ ] Change trash directory
  - [ ] Force create directory
- [ ] File restoration
  - [ ] * functionality
- [x] Clean configuration file
  - [x] Reset directory location 
  - [x] Create missing directories
