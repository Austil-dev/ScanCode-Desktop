<script lang="ts">
  import { onMount } from 'svelte';
  
  let { show = false } = $props<{ show: boolean }>();
  let mounted = $state(false);

  onMount(() => {
    mounted = true;
  });
</script>

{#if show && mounted}
  <div class="transition-overlay" class:show={show}>
    <div class="transition-content">
      <div class="spinner-container">
        <div class="spinner"></div>
        <div class="spinner-ring"></div>
      </div>
      <p class="transition-text">Redirection...</p>
    </div>
  </div>
{/if}

<style>
  .transition-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 9999;
    opacity: 0;
    animation: fadeIn 0.3s ease forwards;
  }

  .transition-overlay.show {
    opacity: 1;
  }

  @keyframes fadeIn {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  .transition-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 30px;
  }

  .spinner-container {
    position: relative;
    width: 80px;
    height: 80px;
  }

  .spinner {
    position: absolute;
    width: 80px;
    height: 80px;
    border: 4px solid transparent;
    border-top-color: #d4af37;
    border-right-color: #d4af37;
    border-radius: 50%;
    animation: spin 1s cubic-bezier(0.68, -0.55, 0.265, 1.55) infinite;
  }

  .spinner-ring {
    position: absolute;
    width: 60px;
    height: 60px;
    top: 10px;
    left: 10px;
    border: 3px solid transparent;
    border-bottom-color: #667eea;
    border-left-color: #667eea;
    border-radius: 50%;
    animation: spin 1.5s cubic-bezier(0.68, -0.55, 0.265, 1.55) infinite reverse;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  .transition-text {
    color: #e0e0e0;
    font-size: 1.2rem;
    font-weight: 500;
    letter-spacing: 1px;
    animation: pulse 1.5s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% {
      opacity: 0.6;
    }
    50% {
      opacity: 1;
    }
  }
</style>
