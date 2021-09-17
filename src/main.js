import './global.css';
import 'regenerator-runtime/runtime'

import App from './App.svelte';

const app = new App({
	target: document.body,
	props: {
		name: 'world'
	}
});

export default app;
