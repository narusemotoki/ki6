const electron = require('electron')

electron.app.whenReady().then(() => {
  new electron.BrowserWindow().loadFile('index.html')
})
