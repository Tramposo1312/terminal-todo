# Feature Roadmap for Rust Terminal TODO App

## Core Functionality

1. **List Management**
   - [x] Add new items to the TODO list
   - [x] Move items from TODO to DONE
   - [x] Move items back from DONE to TODO
   - [x] Delete items from either list
   - [x] Edit existing items

2. **Navigation**
   - [x] Navigate through TODO list
   - [x] Navigate through DONE list
   - [x] Switch focus between TODO and DONE lists

3. **Persistence**
   - [x] Save TODO and DONE lists to a file
   - [x] Load TODO and DONE lists from a file on startup

## User Experience Enhancements

4. **UI Improvements**
   - [ ] Add a status bar (current mode, item count, etc.)
   - [ ] Implement scrolling for lists that exceed screen height
   - [ ] Add visual indicators for current focus (TODO or DONE list)

5. **Keybindings and Commands**
   - [ ] Implement a command mode (similar to Vim)
   - [ ] Allow customizable keybindings
   - [ ] Add a help screen showing all available commands

6. **Item Properties**
   - [ ] Add due dates to TODO items
   - [ ] Implement priority levels for TODO items
   - [ ] Add tags or categories to items

## Advanced Features

7. **Filtering and Sorting**
   - [ ] Filter items based on tags, priority, or due date
   - [ ] Sort items based on various criteria (alphabetical, priority, due date)

8. **Multi-user Support**
   - [ ] Implement user accounts
   - [ ] Allow sharing of TODO lists between users

9. **Undo/Redo Functionality**
   - [ ] Implement an undo stack for actions
   - [ ] Allow undoing and redoing actions

10. **Search Functionality**
    - [ ] Implement a search feature to find items quickly

11. **Data Export/Import**
    - [ ] Export TODO and DONE lists to common formats (CSV, JSON)
    - [ ] Import TODO lists from other formats

12. **Notifications**
    - [ ] Implement a system for reminders based on due dates
    - [ ] Integrate with system notifications (if running as a daemon)