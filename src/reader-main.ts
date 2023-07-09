import { createApp } from 'vue';
import reader from './reader.vue';
// import router from './router';
// import store from './store';
// import 'ant-design-vue/dist/antd.css';
import 'ant-design-vue/dist/reset.css';

import * as config from './config';

const Application = createApp(reader);

config.registerReaderComponents(Application);
Application.mount('#app');
