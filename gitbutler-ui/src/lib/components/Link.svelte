<script lang="ts">
	import Icon from '$lib/components/Icon.svelte';
	import { openExternalUrl } from '$lib/utils/url';
	import { onMount } from 'svelte';

	let classes = '';
	export { classes as class };
	export let target: '_blank' | '_self' | '_parent' | '_top' | undefined = undefined;
	export let rel: string | undefined = undefined;
	export let role: 'basic' | 'primary' | 'error' = 'basic';
	export let disabled = false;
	export let href: string | undefined = undefined;

	let element: HTMLAnchorElement | HTMLButtonElement | undefined;

	onMount(() => {
		if (element) {
			element.ariaLabel = element.innerText?.trim();
		}
	});

	$: isExternal = href?.startsWith('http');
</script>

{#if href}
	<a
		{href}
		{target}
		{rel}
		class="link inline-flex cursor-pointer items-center justify-center gap-1 whitespace-nowrap hover:underline hover:ease-in {role} {classes}"
		bind:this={element}
		class:disabled
		on:click={(e) => {
			if (href && isExternal) {
				e.preventDefault();
				e.stopPropagation();
				openExternalUrl(href);
			}
		}}
	>
		<div class="truncate">
			<slot />
		</div>
		{#if isExternal}
			<div class="shrink-0">
				<Icon name="open-link" />
			</div>
		{/if}
	</a>
{/if}
