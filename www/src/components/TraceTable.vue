<script setup lang="ts">
import { Step } from "@caie-assembler/pkg";

defineProps<{ steps: Step[]; highlight: number }>();
const emit = defineEmits<{ (e: "update:highlight", address: number): void }>();
</script>

<template>
  <table
    class="table table-bordered table-sm text-center"
    style="table-layout: fixed"
  >
    <thead>
      <tr>
        <th>PC</th>
        <th>ACC</th>
        <th>IX</th>
        <th>OUT</th>
      </tr>
    </thead>
    <tbody>
      <tr
        v-for="(step, i) in steps"
        :key="i"
        :class="{ 'table-active': highlight === step.pc }"
        @mouseover="() => $emit('update:highlight', step.pc)"
        @mouseleave="() => emit('update:highlight', 0)"
      >
        <td>{{ step.pc }}</td>
        <td>{{ step.acc }}</td>
        <td>{{ step.ix }}</td>
        <td>{{ step.out }}</td>
      </tr>
    </tbody>
  </table>
</template>
