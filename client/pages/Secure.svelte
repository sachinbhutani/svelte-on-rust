<script>
    let secureContent='';
    import {onMount} from 'svelte';
    onMount( async() => {
        const res = await fetch('/api/secure',{credentials: 'same-origin'});
        secureContent = await res.json();
    })
</script>
<div class="content">

<p>
    This is secret secure area!! <br>
    Here's the secret message from the server: 
    ( if you are not logged in, result should be "error")<br>
    <strong>Result: </strong>{secureContent.result} <br>
    <strong>Message: </strong>{secureContent.message} <br>
</p>
</div>