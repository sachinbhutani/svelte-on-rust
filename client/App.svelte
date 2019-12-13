<script>
	// export let name;
	import Nav from './Nav.svelte';
	import Footer from './Footer.svelte';
	import {Router, Route} from 'yrv';
	import Home from './pages/Home.svelte';
	import About from './pages/About.svelte';
	import Common from './pages/Common.svelte';
	import Catcher from './pages/Catcher.svelte';
</script>

<main>
  <Nav></Nav>
  <Router>
    <Route exact path="/"><Home></Home></Route>
    <Route path="#about"><About></About></Route>
	<Route path="#app/common" let:router><Common></Common></Route>
	<Route path="#:name" let:router><Catcher></Catcher></Route> 
  </Router> 
  <Footer></Footer>
</main>