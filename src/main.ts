// SPDX-License-Identifier: GPL-3.0-or-later
// Arion - Your space for thought.
// Copyright (C) 2026 Abdallah

import { mount } from 'svelte';
import App from './App.svelte';
import "./global.css";

const app = mount(App, {
  target: document.getElementById('app')!,
});

export default app;
