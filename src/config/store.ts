import { app } from 'electron';
import Store from 'electron-store';
import path from 'path';

const store = new Store({
  cwd: path.join(app.getPath('userData'), 'config'),
});
export { store };
