<script>
  import { onMount, onDestroy } from 'svelte';
  import { ParticleSystem } from './particles.js';
  import clearIcon from '../assets/Clear.svg?raw';
  import dockIcon from '../assets/Dock.svg?raw';
  import undockIcon from '../assets/Undock.svg?raw';
  import removeIcon from '../assets/Remove.svg?raw';
  import unselectIcon from '../assets/Unselect.svg?raw';

  let {
    items = [],
    selections = [],
    filter = '',
    mode = 'docked',
    latestOutput = null,
    accentColor = '#263ed2',
    particlesEnabled = true,
    generationTrigger = 0,
    onClear = () => {},
    onRemoveSelected = () => {},
    onUnselectAll = () => {},
    onToggleDock = () => {},
    onToggleSelect = () => {},
    onFilterChange = () => {},
    soundFiles = {},
  } = $props();

  let canvasRef = $state(null);
  let particleSystem = null;
  let resizeObserver = null;

  function handleFilterKeydown(e) {
    if (e.key === 'Escape') {
      onFilterChange('');
    }
  }

  function handleItemKeydown(e, origIndex, item) {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      const items = Array.from(e.currentTarget.parentElement.querySelectorAll('.list-item'));
      const pos = items.indexOf(e.currentTarget);
      const nextIndex = Math.min(pos + 1, items.length - 1);
      items[nextIndex]?.focus();
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      const items = Array.from(e.currentTarget.parentElement.querySelectorAll('.list-item'));
      const pos = items.indexOf(e.currentTarget);
      const prevIndex = Math.max(pos - 1, 0);
      items[prevIndex]?.focus();
    } else if (e.key === 'Enter' || e.key === ' ') {
      onToggleSelect(item);
      e.preventDefault();
    }
  }

  onMount(() => {
    setTimeout(() => {
      if (!canvasRef || !canvasRef.parentElement) return;
      const rect = canvasRef.parentElement.getBoundingClientRect();
      if (rect.width > 0 && rect.height > 0) {
        canvasRef.width = rect.width;
        canvasRef.height = rect.height;
        particleSystem = new ParticleSystem(canvasRef);
        resizeObserver = new ResizeObserver(() => {
          if (canvasRef && canvasRef.parentElement) {
            const r = canvasRef.parentElement.getBoundingClientRect();
            canvasRef.width = r.width;
            canvasRef.height = r.height;
          }
        });
        resizeObserver.observe(canvasRef.parentElement);
      }
    }, 100);
  });

  onDestroy(() => {
    if (resizeObserver) resizeObserver.disconnect();
    if (particleSystem) particleSystem.destroy();
  });

  $effect(() => {
    // Track the trigger; the body only runs when it changes.
    if (generationTrigger === 0) return;
    if (!particlesEnabled) return;
    if (!particleSystem || !canvasRef) return;
    particleSystem.gentleRise(150, accentColor);
    particleSystem.startAnimation();
  });
</script>

{#if mode === 'standalone'}
  <div class="standalone-panel">
    <div class="sidebar-header">
      <span class="sidebar-title">Outputs</span>
      <div class="sidebar-actions">
        {#if selections.length > 0}
          <button class="icon-btn icon-remove" onclick={onRemoveSelected} title="Delete selected ({selections.length})">{@html removeIcon}</button>
          <button class="icon-btn icon-unselect" onclick={onUnselectAll} title="Unselect all">{@html unselectIcon}</button>
        {:else}
          <button class="icon-btn icon-clear" onclick={onClear} title="Clear all">{@html clearIcon}</button>
        {/if}
        <button class="icon-btn icon-dock" onclick={() => { try { new Audio(soundFiles.dockSound).play(); } catch {} onToggleDock(); }} title="Dock">{@html dockIcon}</button>
      </div>
    </div>
    {#if items.length > 5}
      <input type="text" class="filter-input" placeholder="Filter..." value={filter} oninput={(e) => onFilterChange(e.target.value)} onkeydown={handleFilterKeydown} />
    {/if}
    <div class="list-wrapper">
      <div class="output-list">
        {#each items as { value: item, index: origIndex } (origIndex)}
          <div class="list-item" class:latest-output-item={origIndex === items.length - 1}
            class:selected={selections.includes(item)} data-index={origIndex} data-value={item}
            onclick={(e) => { if (e.target.classList.contains('checkmark')) return; onToggleSelect(item); }}
            onkeydown={(e) => handleItemKeydown(e, origIndex, item)}
            tabindex="0"
            style="border-left: 3px solid var(--accent);">
            <span class="checkmark-wrap"><input type="checkbox" class="checkmark" onclick={(e) => { e.stopPropagation(); onToggleSelect(item); }} checked={selections.includes(item)} /></span>
            <span class="item-number">{origIndex + 1}.</span>
            <span class="item-text">{item}</span>
          </div>
        {:else}
          <div class="empty-state">No outputs yet</div>
        {/each}
      </div>
    </div>
    <canvas class="output-particle-canvas" bind:this={canvasRef}></canvas>
  </div>
{:else}
  <div class="sidebar">
    <div class="sidebar-header">
      <span class="sidebar-title">Outputs</span>
      <div class="sidebar-actions">
        {#if selections.length > 0}
          <button class="icon-btn icon-remove" onclick={onRemoveSelected} title="Delete selected ({selections.length})">{@html removeIcon}</button>
          <button class="icon-btn icon-unselect" onclick={onUnselectAll} title="Unselect all">{@html unselectIcon}</button>
        {:else}
          <button class="icon-btn icon-clear" onclick={onClear} title="Clear all">{@html clearIcon}</button>
        {/if}
        <button class="icon-btn icon-undock" onclick={onToggleDock} title="Undock">{@html undockIcon}</button>
      </div>
    </div>
    {#if items.length > 5}
      <input type="text" class="filter-input" placeholder="Filter..." value={filter} oninput={(e) => onFilterChange(e.target.value)} onkeydown={handleFilterKeydown} />
    {/if}
    <div class="list-wrapper">
      <div class="output-list">
        {#each items as { value: item, index: origIndex } (origIndex)}
          <div class="list-item" class:latest-output-item={origIndex === items.length - 1}
            class:selected={selections.includes(item)} data-index={origIndex} data-value={item}
            onclick={(e) => { if (e.target.classList.contains('checkmark')) return; onToggleSelect(item); }}
            onkeydown={(e) => handleItemKeydown(e, origIndex, item)}
            tabindex="0"
            style="border-left: 3px solid var(--accent);">
            <span class="checkmark-wrap"><input type="checkbox" class="checkmark" onclick={(e) => { e.stopPropagation(); onToggleSelect(item); }} checked={selections.includes(item)} /></span>
            <span class="item-number">{origIndex + 1}.</span>
            <span class="item-text">{item}</span>
          </div>
        {:else}
          <div class="empty-state">No outputs yet</div>
        {/each}
      </div>
    </div>
    <canvas class="output-particle-canvas" bind:this={canvasRef}></canvas>
  </div>
{/if}
