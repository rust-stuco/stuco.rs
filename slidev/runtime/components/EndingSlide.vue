<script setup lang="ts">
import ferris from '../../../lectures/images/ferris_happy.svg'

defineProps<{ nextLecture?: string }>()
</script>

<template>
  <div
    class="slidev-layout ending-slide image-right"
    :class="nextLecture ? 'image-width-30' : 'image-width-35'"
  >
    <h1>
      <strong v-if="nextLecture">Next Lecture: {{ nextLecture }}</strong>
      <strong v-else>Thanks for taking Rust StuCo!</strong>
    </h1>

    <img
      class="slide-image"
      :style="{ '--image-size': nextLecture ? 0.8 : 0.9 }"
      :src="ferris"
      alt="Happy Ferris the crab"
    >

    <template v-if="nextLecture">
      <p>Thanks for coming!</p>
      <p>Please stay seated to fill out the attendance form.</p>
    </template>

    <p class="credits">
      <em v-if="nextLecture">Slides created by:</em>
      <em v-else>Rust StuCo (98-008) has been created by:</em><br>
      Connor Tsui, Benjamin Owad, David Rudo,<br>
      Jessica Ruan, Fiona Fisher, Terrance Chen,<br>
      Stephen Mao, Hugo Latendresse,<br>
      Anish Pallati, Max Wen
    </p>
  </div>
</template>

<style scoped>
.slidev-layout.ending-slide {
  position: absolute;
  inset: 0;
}

.credits {
  margin-top: 32px;
  font-size: 22px;
}
</style>
