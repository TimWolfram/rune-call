import App from './App.vue'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { registerPlugins } from '@/plugins'

const pinia = createPinia();
const app = createApp(App);

app.use(pinia);
registerPlugins(app);

app.mount('#app');