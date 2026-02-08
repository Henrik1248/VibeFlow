<script setup>
import { ref, reactive, onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

const amplitude = ref(0);
// 3 stripes - Using reactive array for better performance in animation loop
const dots = reactive([
    { height: 8, opacity: 0.8 },
    { height: 8, opacity: 0.8 },
    { height: 8, opacity: 0.8 }
]);

let unlistenAmp;
let animationFrame;
let lastTime = 0;

onMounted(async () => {
    invoke('ui_ready').catch(e => console.error("Failed to invoke ui_ready:", e));

    unlistenAmp = await listen('amplitude', (event) => {
        // Boost amplitude for more dramatic effect
        const val = event.payload;
        if (Math.random() < 0.05) console.log("Amp received:", val); // Log 5% of packets to avoid spam
        // UNCAPPED: Let it go wild. Sensitivity 60x.
        amplitude.value = val * 60.0;
    });

    const animate = (time) => {
        const delta = time - lastTime;
        lastTime = time;

        // Directly mutate reactive objects
        dots.forEach((dot, i) => {
            const isActive = amplitude.value > 0.001;
            
            // Idle: tiny dots (6px) breathing slightly
            const idleHeight = 6 + Math.sin(time * 0.003 + i) * 2;
            
            // Active: High energy vertical expansion (up to 300px limit naturally by window)
            const waveFactor = 0.5 + 0.5 * Math.sin(time * 0.01 + i * 2.0);
            const activeHeight = 6 + (amplitude.value * 250) * waveFactor;
            
            const targetHeight = isActive ? activeHeight : idleHeight;
            
            // Spring-like damping
            dot.height = dot.height + (targetHeight - dot.height) * 0.25;
            dot.opacity = 0.9;
        });

        animationFrame = requestAnimationFrame(animate);
    };
    
    animationFrame = requestAnimationFrame(animate);
});

onUnmounted(() => {
    if (unlistenAmp) unlistenAmp();
    if (animationFrame) cancelAnimationFrame(animationFrame);
});
</script>

<template>
  <div class="overlay-wrapper">
      <div class="glass-capsule">
        <div class="stripes">
          <div v-for="(dot, i) in dots" :key="i" 
               class="stripe"
               :style="{ 
                 height: `${dot.height}px`
               }">
          </div>
        </div>
      </div>
  </div>
</template>

<style>
/* Global enforcement for this component's view */
html, body, #app {
    background: transparent !important;
    overflow: hidden;
}
</style>

<style scoped>
.overlay-wrapper {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center; 
  justify-content: center;
  background: transparent !important;
  pointer-events: none;
}

.glass-capsule {
    /* White Transparent Glass */
    background: rgba(255, 255, 255, 0.15); /* Slightly visible white tint */
    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
    border: 1px solid rgba(255, 255, 255, 0.2);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
    
    border-radius: 99px; /* Fully rounded pill */
    padding: 10px 18px; /* Tighter padding */
    
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
}

.stripes {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  background: transparent;
}

.stripe {
  width: 6px;
  background: #ffffff; /* White dots for contrast on glass */
  border-radius: 99px;
  will-change: height;
  box-shadow: 0 0 10px rgba(255, 255, 255, 0.4);
}
</style>
