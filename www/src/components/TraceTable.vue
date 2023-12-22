<script setup lang="ts">
import { Step } from "@caie-assembler/pkg";

defineProps<{ steps: Step[] }>();
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
      <tr v-for="(step, i) in steps" :key="i">
        <td>{{ step.pc }}</td>
        <td>{{ step.acc }}</td>
        <td>{{ step.ix }}</td>
        <td>{{ step.out }}</td>
      </tr>
    </tbody>
  </table>
</template>
