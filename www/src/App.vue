<script setup lang="ts">
import AssemblyEditor from "./components/AssemblyEditor.vue";
import { Assembler, Location } from "@caie-assembler/pkg";
import TraceTable from "./components/TraceTable.vue";

const steps = Assembler.from_memory([
  new Location(200, "", "LDD", "365"),
  new Location(201, "", "CMP", "366"),
  new Location(202, "", "JPE", "209"),
  new Location(203, "", "INC", "ACC"),
  new Location(204, "", "STO", "365"),
  new Location(205, "", "MOV", "IX"),
  new Location(206, "", "LDX", "365"),
  new Location(207, "", "OUT", ""),
  new Location(208, "", "JMP", "200"),
  new Location(209, "", "END", ""),
  new Location(365, "", "", "1"),
  new Location(366, "", "", "3"),
  new Location(367, "", "", "65"),
  new Location(368, "", "", "66"),
]).execute(200);
</script>

<template>
  <main>
    <AssemblyEditor />
    <TraceTable :steps="steps" />
  </main>
</template>
