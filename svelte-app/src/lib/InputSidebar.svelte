<script>
  import { tick } from 'svelte';
  import clearIcon from '../assets/Clear.svg?raw';
  import saveIcon from '../assets/Save.svg?raw';
  import importIcon from '../assets/Import.svg?raw';
  import dockIcon from '../assets/Dock.svg?raw';
  import undockIcon from '../assets/Undock.svg?raw';
  import addIcon from '../assets/Add.svg?raw';
  import removeIcon from '../assets/Remove.svg?raw';
  import unselectIcon from '../assets/Unselect.svg?raw';
  import dragIndicatorIcon from '../assets/DragIndicator.svg?raw';

  let {
    items = [],
    selections = [],
    filter = '',
    editingIndex = -1,
    editValue = '',
    docked = true,
    mode = 'docked',
    dragAction = null,
    onAdd = () => {},
    onClear = () => {},
    onRemoveSelected = () => {},
    onUnselectAll = () => {},
    onSave = () => {},
    onLoad = () => {},
    onToggleDock = () => {},
    onStartEditing = () => {},
    onSaveEdit = () => {},
    onCancelEdit = () => {},
    onToggleSelect = () => {},
    onFilterChange = () => {},
    soundFiles = {},
  } = $props();

  let inputField = $state(null);

  async function handleKeydown(e) {
    if (e.key === 'Enter') {
      const value = e.target.value;
      if (value.trim()) {
        await onAdd(value);
        e.target.value = '';
      }
    }
  }

  function handleFilterKeydown(e) {
    if (e.key === 'Escape') {
      onFilterChange('');
    }
  }

  function handleItemKeydown(e, origIndex) {
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
    } else if ((e.key === 'Enter' || e.key === ' ') && !e.target.classList.contains('checkmark')) {
      onToggleSelect(origIndex);
      e.preventDefault();
    }
  }

  function handleEditKeydown(e) {
    if (e.key === 'Enter') {
      onSaveEdit();
      e.preventDefault();
    } else if (e.key === 'Escape') {
      onCancelEdit();
      e.preventDefault();
    }
  }

  async function handleAddClick() {
    if (inputField?.value.trim()) {
      await onAdd(inputField.value);
      inputField.value = '';
    } else {
      inputField?.focus();
    }
  }
</script>

{#if mode === 'standalone'}
  <div class="standalone-panel">
    <div class="sidebar-header">
      <span class="sidebar-title">Inputs</span>
      <div class="sidebar-actions">
        {#if selections.length > 0}
          <button class="icon-btn icon-remove" onclick={onRemoveSelected} title="Delete selected ({selections.length})">{@html removeIcon}</button>
          <button class="icon-btn icon-unselect" onclick={onUnselectAll} title="Unselect all">{@html unselectIcon}</button>
        {:else}
          <button class="icon-btn icon-clear" onclick={onClear} title="Clear all">{@html clearIcon}</button>
        {/if}
        {#if selections.length === 0}
          <button class="icon-btn icon-add" onclick={handleAddClick} title="Add">{@html addIcon}</button>
        {/if}
        <button class="icon-btn icon-dock" onclick={() => { try { new Audio(soundFiles.dockSound).play(); } catch {} onToggleDock(); }} title="Dock">{@html dockIcon}</button>
      </div>
    </div>
    {#if items.length > 5}
      <input type="text" class="filter-input" placeholder="Filter..." value={filter} oninput={(e) => onFilterChange(e.target.value)} onkeydown={handleFilterKeydown} />
    {/if}
    <div class="list-wrapper" use:dragAction={'input'}>
      <div class="input-list">
        {#each items as { value: item, index: origIndex } (origIndex)}
          {#if editingIndex === origIndex}
            <div class="list-item edit-mode" onclick={(e) => e.stopPropagation()}>
              <input type="text" class="edit-input" value={editValue} autofocus
                oninput={(e) => { editValue = e.target.value; }}
                onkeydown={handleEditKeydown} />
              <button class="edit-cancel" onclick={() => { onCancelEdit(); }}>✕</button>
            </div>
          {:else}
            <div class="list-item" class:selected={selections.includes(origIndex)}
              data-index={origIndex}
              onclick={(e) => { if (e.target.classList.contains('checkmark')) return; if (e.currentTarget.dataset.dragAttempted === 'true') { e.currentTarget.dataset.dragAttempted = 'false'; return; } onToggleSelect(origIndex); }}
              onkeydown={(e) => handleItemKeydown(e, origIndex)}
              tabindex="0">
              <span class="drag-handle" title="Drag to reorder">{@html dragIndicatorIcon}</span>
              <span class="checkmark-wrap"><input type="checkbox" class="checkmark" onclick={(e) => { e.stopPropagation(); onToggleSelect(origIndex); }} checked={selections.includes(origIndex)} /></span>
              <span class="item-number">{origIndex + 1}.</span>
              <span class="item-text">{item}</span>
              <span class="edit-link" onclick={(e) => { e.stopPropagation(); onStartEditing(origIndex, item); }}>Edit</span>
            </div>
          {/if}
        {:else}
          <div class="empty-state">No inputs added yet</div>
        {/each}
      </div>
    </div>
    <div class="add-input-container">
      <input bind:this={inputField} type="text" class="add-input" placeholder="Type and press Enter..."
        onkeydown={handleKeydown} />
    </div>
  </div>
{:else}
  <div class="sidebar">
    <div class="sidebar-header">
      <span class="sidebar-title">Inputs</span>
      <div class="sidebar-actions">
        {#if selections.length > 0}
          <button class="icon-btn icon-remove" onclick={onRemoveSelected} title="Delete selected ({selections.length})">{@html removeIcon}</button>
          <button class="icon-btn icon-unselect" onclick={onUnselectAll} title="Unselect all">{@html unselectIcon}</button>
        {:else}
          <button class="icon-btn icon-clear" onclick={onClear} title="Clear all">{@html clearIcon}</button>
        {/if}
        {#if selections.length === 0}
          <button class="icon-btn icon-add" onclick={handleAddClick} title="Add">{@html addIcon}</button>
        {/if}
        <button class="icon-btn icon-save" onclick={onSave} title="Save inputs">{@html saveIcon}</button>
        <button class="icon-btn icon-load" onclick={onLoad} title="Load inputs">{@html importIcon}</button>
        <button class="icon-btn icon-undock" onclick={onToggleDock} title="Undock">{@html undockIcon}</button>
      </div>
    </div>
    {#if items.length > 5}
      <input type="text" class="filter-input" placeholder="Filter..." value={filter} oninput={(e) => onFilterChange(e.target.value)} onkeydown={handleFilterKeydown} />
    {/if}
    <div class="list-wrapper" use:dragAction={'input'}>
      <div class="input-list">
        {#each items as { value: item, index: origIndex } (origIndex)}
          {#if editingIndex === origIndex}
            <div class="list-item edit-mode" onclick={(e) => e.stopPropagation()}>
              <input type="text" class="edit-input" value={editValue} autofocus
                oninput={(e) => { editValue = e.target.value; }}
                onkeydown={handleEditKeydown} />
              <button class="edit-cancel" onclick={() => { onCancelEdit(); }}>✕</button>
            </div>
          {:else}
            <div class="list-item" class:selected={selections.includes(origIndex)}
              data-index={origIndex}
              onclick={(e) => { if (e.target.classList.contains('checkmark')) return; if (e.currentTarget.dataset.dragAttempted === 'true') { e.currentTarget.dataset.dragAttempted = 'false'; return; } onToggleSelect(origIndex); }}
              onkeydown={(e) => handleItemKeydown(e, origIndex)}
              tabindex="0">
              <span class="drag-handle" title="Drag to reorder">{@html dragIndicatorIcon}</span>
              <span class="checkmark-wrap"><input type="checkbox" class="checkmark" onclick={(e) => { e.stopPropagation(); onToggleSelect(origIndex); }} checked={selections.includes(origIndex)} /></span>
              <span class="item-number">{origIndex + 1}.</span>
              <span class="item-text">{item}</span>
              <span class="edit-link" onclick={(e) => { e.stopPropagation(); onStartEditing(origIndex, item); }}>Edit</span>
            </div>
          {/if}
        {:else}
          <div class="empty-state">No inputs added yet</div>
        {/each}
      </div>
    </div>
    <div class="add-input-container">
      <input bind:this={inputField} type="text" class="add-input" placeholder="Type and press Enter..."
        onkeydown={handleKeydown} />
    </div>
  </div>
{/if}

<style>
  .edit-input {
    background-color: var(--bg-primary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
    padding: 7px;
    font-size: 13px;
    width: 100%;
    box-sizing: border-box;
    outline: none;
  }

  .edit-cancel {
    background: transparent;
    border: none;
    color: var(--text-secondary);
    cursor: pointer;
    font-size: 14px;
    padding: 0 6px;
    opacity: 0.7;
  }

  .edit-cancel:hover {
    opacity: 1;
    color: var(--accent);
  }
</style>
