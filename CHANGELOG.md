## [0.5.0] - 2026-02-08

### 🚀 Features

- *(window)* App now remembers window size
- *(window)* App now remembers maximized window
- Add language support
- *(translations)* Added dutch
- *(about)* Added credits section

### 🐛 Bug Fixes

- *(web-app-view)* Set category default to Network / Internet

### 💼 Other

- *(deps)* Bump bytes in the cargo group across 1 directory (#22)
- *(deps)* Bump git2 in the cargo group across 1 directory (#23)
- *(deps)* Bump time in the cargo group across 1 directory (#24)

### 🚜 Refactor

- *(desktop-file)* Static method for is_owned check
- *(desktop-file)* Remove expect in method
- *(desktop-file)* Revert: set_defaults on construction
- *(desktop-file)* When loading web apps, skip non desktop files
- *(about)* About to own module, added translation support for app menu and about
- *(about)* Revert translation of about

### 📚 Documentation

- *(readme)* Added contributing section + renamed translation dir
- *(readme)* Fix typo

### 🎨 Styling

- *(browsers)* Allow longer method for expand content

### ⚙️ Miscellaneous Tasks

- Print version on info channel
- *(translation)* Actually translate issues to dutch
- *(translations)* Add more_info + fix some issues
- *(about)* Add language to credits
- *(translations)* Add web app category
## [0.4.1] - 2026-02-01

### 🐛 Bug Fixes

- *(desktop-file)* Web apps update again on app update

### ⚙️ Miscellaneous Tasks

- *(release)* V0.4.1
## [0.4.0] - 2026-02-01

### 🚀 Features

- *(icon-picker)* Allow more image types
- *(desktop-file)* Set a default category
- *(desktop-file)* Add description
- *(web-app-view)* Optional settings for desktops with an app menu

### 🐛 Bug Fixes

- *(web-app-view)* Reset button is now disabled after saving a new web app
- *(web-apps)* App list is now sorted by name
- *(web-app-view)* Make sure "No browser" is selected when browser is missing
- *(firefox)* More reliable popups on firefox profile

### 🚜 Refactor

- *(desktop-file)* Keys enum to Key
- *(desktop-file)* Move deps to own files
- Removed all unwraps + more optimizations
- *(app-dirs)* Update dir names
- *(web-app-view)* Optional settings now save on apply

### ⚙️ Miscellaneous Tasks

- *(release)* Fix for last_released_version
- Format
- Update screenshots
- *(release)* V0.4.0
## [0.3.1] - 2026-01-26

### 🐛 Bug Fixes

- *(release)* Corrected last released version
- *(release)* Increment patch version for dry-run
- Use Adwaita icon theme on KDE

### 📚 Documentation

- *(readme)* Update README.md (#14)
- *(description)* Updated description text from #14

### ⚙️ Miscellaneous Tasks

- *(release)* V0.3.1
## [0.3.0] - 2026-01-22

### 🚀 Features

- Show update status + add release notes to about
- *(desktop-file)* Allow local ip as domain

### 🐛 Bug Fixes

- *(icon-picker)* Previous custom icon now shows when online fetch fails
- *(icon-picker)* Currently used icon is now also loaded
- *(web-app-view)* Url validator now also validates local ips

### 🚜 Refactor

- *(desktop-file)* Move validation to url package

### 📚 Documentation

- *(readme)* Added flathub link

### ⚙️ Miscellaneous Tasks

- *(screenshots)* Reorder
- Added copywrite
- *(release)* V0.3.0
