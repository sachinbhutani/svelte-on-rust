<script>
    import svelteLogo from '../assets/svelte.svg'
    import Counter from './Counter.svelte'
    import {APIserver} from './store'
    import {onMount} from 'svelte'
    export let data = {};
    
    onMount( async()=>{
        let api_path = `${$APIserver}`
        try{
            let res = await fetch(api_path);
            data= await res.text()
        }catch(err){
            console.log(err)
        }
    })
</script>

  <div>
    <a href="https://svelte.dev" target="_blank" rel="noreferrer">
      <img src={svelteLogo} class="logo svelte" alt="Svelte Logo" />
    </a>
     <a href="https://rust-lang.org" target="_blank" rel="noreferrer">
      <img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" class="logo" alt="Rust Logo" />
    </a>
  </div>
  <h1>Svelte on Rust</h1>
  <h4>A compiled frontend, deserves a compiled backend.</h4>

  <div class="card">
    <Counter />
  </div>

  <p>
    {data}
  </p>

<style>
  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00aa);
  }
</style>
