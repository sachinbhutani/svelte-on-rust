<script>
    import {Link} from 'yrv';
</script>

<style>
	:global([aria-current]) { font-weight: bold; }
</style>

<Link href="/"> Home </Link> | 
<Link href="#about"> About </Link> | 
<a href="/api"> Server Route - String </a> |
<a href="/api/message"> Server Route - JSON </a> |
<Link href="#app/common?p1=test&p2=3"> Common Route </Link> | 
<Link href="#catch-it"> Catch All </Link>