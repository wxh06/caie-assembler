<script setup lang="ts">
import { Step } from "@caie-assembler/pkg";

defineProps<{ steps: Step[] }>();
</script>

<template>
  <table>
    <thead>
      <tr>
        <th>PC</th>
        <th>ACC</th>
        <th>IX</th>
        <th>OUT</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(step, i) in steps" :key="i">
        <td>{{ step.pc }}</td>
        <td>{{ step.acc }}</td>
        <td>{{ step.ix }}</td>
        <td>{{ step.out }}</td>
      </tr>
    </tbody>
  </table>
</template>

<style>
table {
  border-collapse: collapse;
}

th,
td {
  border: 1px solid black;
}
</style>
