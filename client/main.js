import App from './App.svelte';

const app = new App({
	target: document.body,
	props: {
		name: 'from Svelte'
	}
});

export default app;