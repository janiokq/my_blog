import{h as p,x as f,y as t,z as u,A as l,D as a,G as s,O as _,B as h,Y as n,W as r,T as m}from"./element-plus.935a2e22.js";import{_ as b}from"./index.dcbee9bf.js";const v=p({name:"SelectPage",props:{isShow:{type:Boolean,default:!1},title:{type:String,default:"\u65B0\u7A97\u53E3"}},emits:["update:show"],setup(e,o){return{close:()=>o.emit("update:show",!e.isShow),slots:o.slots}}}),w={key:0,class:"open-select-mask w-full h-full bg-black bg-opacity-30 z-50 fixed top-0 left-0 flex flex-center"},y={class:"open-select max-w-screen-xl bg-white flex flex-col overflow-x-hidden overflow-y-auto",style:{width:"100%",height:"100%"}},x={class:"h-10 flex justify-between items-center px-3 shadow-sm border-b border-gray-100"},g={class:"flex-1 overflow-hidden"},k={key:0,class:"open-select-btn h-12 flex border-t border-gray-100"};function B(e,o,i,S,$,C){const d=f("el-scrollbar");return t(),u(m,{name:"el-fade-in"},{default:l(()=>[e.isShow?(t(),a("div",w,[s("div",y,[s("div",x,[s("span",null,_(e.title),1),s("div",null,[s("i",{class:"el-icon-close cursor-pointer",onClick:o[0]||(o[0]=(...c)=>e.close&&e.close(...c))})])]),s("div",g,[h(d,null,{default:l(()=>[n(e.$slots,"default",{},void 0,!0)]),_:3})]),e.slots.btn?(t(),a("div",k,[n(e.$slots,"btn",{},void 0,!0)])):r("",!0)])])):r("",!0)]),_:3})}var O=b(v,[["render",B],["__scopeId","data-v-00687f48"]]);export{O};
