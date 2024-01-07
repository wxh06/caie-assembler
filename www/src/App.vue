<script setup lang="ts">
import { ref } from "vue";
import { Assembler, type Step } from "@caie-assembler/pkg";
import AssemblyEditor from "./components/AssemblyEditor.vue";
import TraceTable from "./components/TraceTable.vue";

const steps = ref<Step[]>([]);
const highlight = ref<number>(0);

function execute(assembler: Assembler, start: number) {
  if (start) steps.value = assembler.execute(start);
}
</script>

<template>
  <main class="container">
    <div class="row align-items-start">
      <AssemblyEditor
        class="mb-4 col-lg"
        :highlight="highlight"
        @submit="execute"
      />

      <div class="mt-1 col-lg">
        <TraceTable :steps="steps" v-model:highlight="highlight" />
      </div>
    </div>
  </main>
</template>
