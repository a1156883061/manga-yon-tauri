// import Button from 'ant-design-vue/lib/button';
// import 'ant-design-vue/lib/button/style/css';

// import configProvider from 'ant-design-vue/lib/config-provider';

// //  导入Header，Footer，Sider，Content组件
// import layout from 'ant-design-vue/lib/layout';
// import 'ant-design-vue/lib/layout/style/css';

// import menu from 'ant-design-vue/lib/menu';
// import 'ant-design-vue/lib/menu/style/css';

// import card from 'ant-design-vue/lib/card';
// import 'ant-design-vue/lib/card/style/css';

// import spin from 'ant-design-vue/lib/spin';
// import 'ant-design-vue/lib/spin/style/css';

import {
  Button,
  ConfigProvider,
  Layout,
  Menu,
  Card,
  Spin,
  Modal,
} from 'ant-design-vue';

import { App } from 'vue';

function registerComponents(app: App) {
  app.use(Button);
  app.use(ConfigProvider);
  app.use(Layout);
  app.use(Menu);
  app.use(Card);
  app.use(Spin);
  app.use(Modal);
}

export { registerComponents };
