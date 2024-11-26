- ePUB renderer: RTL.
- Metadata view.
- Applications: Notes, Terminal.

- Remove
    * djvu


- cache packages during build process
- figure out installation to kobo device process
- package up emulator as standalone app
- remove "flush" option - flush the database on events so that the user doesnt need to do it manually
- let there be a local database (.reading_states)
    - sync this database with database on a server through an API
    - somehow identify latest changes in local database and only sync those