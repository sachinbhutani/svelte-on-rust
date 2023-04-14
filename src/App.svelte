<script>
    import {Router,Route} from 'yrv'
    import {APIserver,user} from './lib/store'
    import Home from './lib/Home.svelte'
    import About from './lib/About.svelte'
    import Common from './lib/Common.svelte'
    import Nav from './lib/Nav.svelte';    
    import Footer from './lib/Footer.svelte';
    import Login from './lib/Login.svelte';
    import Secret from './lib/Secret.svelte';
    let currentUser = "_";
    $: currentUser = $user;

</script>

<main>
    <Nav></Nav>
    {#if currentUser !== "_"}
    <p> Welcome {currentUser}</p>
    {/if}
     <Router>
    <Route exact path="/" component={Home}></Route>
    <Route exact path="/about"><About></About></Route>
	<Route path="/common"><Common></Common></Route>
    <Route path="/secure/secret"><Secret></Secret></Route>
    <Route path="/auth" component={Login}></Route>
  </Router> 
    <Footer></Footer>
</main>
