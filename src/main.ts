import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
// import 'ant-design-vue/dist/antd.css';
import 'ant-design-vue/dist/reset.css';

import * as config from './config';

const Application = createApp(App);

config.registerComponents(Application);
Application.use(router).mount('#app');
