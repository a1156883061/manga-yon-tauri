import { Button, Form, Input, Modal, Select } from 'ant-design-vue';

import { App } from 'vue';

function registerReaderComponents(app: App) {
  app.use(Button);
  app.use(Modal);
  app.use(Form);
  app.use(Select);
}

export { registerReaderComponents };
