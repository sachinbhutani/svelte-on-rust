<script>
    import {Link} from 'yrv';
</script>

<style>
	:global([aria-current]) { font-weight: bold; }
	nav {
		border-bottom: 1px solid rgba(255,62,0,0.1);
		font-weight: 300;
		padding: 0 1em;
	}
	ul {
		margin: 0;
		padding: 0;
	}
	/* clearfix */
	ul::after {
		content: '';
		display: block;
		clear: both;
	}
	li {
		display: block;
		float: left;
	}
	.selected {
		position: relative;
		display: inline-block;
	}
	.selected::after {
		position: absolute;
		content: '';
		width: calc(100% - 1em);
		height: 2px;
		background-color: rgb(255,62,0);
		display: block;
		bottom: -1px;
	}
</style>

<Link href="/"> Home </Link> | 
<Link href="#about"> About </Link> | 
<a href="/api"> Server Route - String </a> |
<a href="/api/message"> Server Route - JSON </a> |
<Link href="#app/common?p1=test&p2=3"> Common Route </Link> | 
<Link href="#catch-it"> Catch All </Link>