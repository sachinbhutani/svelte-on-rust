<script>
	// export let name;
	import Nav from './Nav.svelte';
	import Footer from './Footer.svelte';
	import {Router, Route} from 'yrv';
	import Home from './pages/Home.svelte';
	import About from './pages/About.svelte';
	import Common from './pages/Common.svelte';
	import Catcher from './pages/Catcher.svelte';
	import Login from './pages/Login.svelte';
	import Logout from './pages/Logout.svelte';
	import Secure from './pages/Secure.svelte';
	import {user} from './store.js';
	import {onMount} from 'svelte';
	onMount( async() => {
        const res = await fetch('/api/checkuser',{credentials: 'same-origin'});
        let stateResponse = await res.json();
        if (stateResponse.user_id !== '_') {
            user.set(stateResponse.user_id);
		}else 
		{
			user.set('');
		}
    });
</script>

<main>
  <Nav></Nav>
  <Router>
    <Route exact path="/"><Home></Home></Route>
    <Route path="#about"><About></About></Route>
	<Route path="#app/common" let:router><Common></Common></Route>
	<Route path="#:name" let:router><Catcher></Catcher></Route> 
	<Route exact path="#login"><Login></Login></Route>
	<Route exact path="#logout"><Logout></Logout></Route>
	<Route exact path="#secure"><Secure></Secure></Route>
  </Router> 
  <Footer></Footer>
</main>