import "clsx";
function PageTransition($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { show = false } = $$props;
    {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]-->`);
  });
}
export {
  PageTransition as P
};
