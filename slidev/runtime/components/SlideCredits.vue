<script setup lang="ts">
withDefaults(defineProps<{ label?: string }>(), {
  label: 'Slides created by:',
})
</script>

<template>
  <p>
    <em>{{ label }}</em><br>
    Connor Tsui, Benjamin Owad, David Rudo,<br>
    Jessica Ruan, Fiona Fisher, Terrance Chen,<br>
    Stephen Mao, Hugo Latendresse,<br>
    Anish Pallati, Max Wen
  </p>
</template>
