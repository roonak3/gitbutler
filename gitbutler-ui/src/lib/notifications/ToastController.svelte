<script lang="ts" context="module">
</script>

<script lang="ts">
	import InfoMessage from '$lib/components/InfoMessage.svelte';
	import { dismissToast, toastStore } from '$lib/notifications/toasts';
	import { marked } from 'marked';
	import { slide } from 'svelte/transition';

	var renderer = new marked.Renderer();
	renderer.link = function (href, title, text) {
		if (!title) title = text;
		return '<a target="_blank" href="' + href + '" title="' + title + '">' + text + '</a>';
	};
</script>

<div class="toast-controller">
	{#each $toastStore as toast (toast.id)}
		<div transition:slide={{ duration: 170 }}>
			<InfoMessage
				title={toast.title}
				style={toast.style ?? 'neutral'}
				secondary="Dismiss"
				on:secondary={() => dismissToast(toast.id)}
				shadow>{@html marked.parse(toast.message, { renderer })}</InfoMessage
			>
		</div>
	{/each}
</div>

<style lang="postcss">
	.toast-controller {
		position: absolute;
		display: flex;
		flex-direction: column;
		bottom: var(--space-20);
		right: var(--space-20);
		gap: var(--space-8);
		z-index: 50;
	}
</style>
