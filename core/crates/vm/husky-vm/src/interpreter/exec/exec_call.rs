use husky_entity_route::EntityRoutePtr;

use super::*;

impl<'temp, 'eval: 'temp> Interpreter<'temp, 'eval> {
    pub(super) fn call_specific_routine(
        &mut self,
        f: __LinkageFp,
        nargs: u8,
        output_ty: EntityRoutePtr,
        discard: bool,
    ) -> __VMResult<()> {
        let mut arguments = self.stack.drain(nargs).collect::<Vec<_>>();
        for argument in arguments.iter() {
            if self.stack.len() > 0 {
                assert_ne!(argument.vtable as *const _, unsafe {
                    &__VOID_VTABLE as *const _
                });
            }
            match argument.data_kind() {
                __RegisterDataKind::Moved | __RegisterDataKind::Unreturned => panic!(),
                __RegisterDataKind::Undefined => todo!(),
                _ => (),
            }
        }
        let output = f.call_catch_unwind(self.opt_ctx, arguments)?;
        if !discard {
            self.stack.push(output);
        }
        Ok(())
    }

    pub(super) fn call_interpreted(
        &mut self,
        sheet: &InstructionSheet,
        nargs: u8,
        has_this: bool,
        discard: bool,
    ) -> __VMResult<()> {
        let mut interpreter = Interpreter::new(
            self.db,
            self.opt_ctx,
            self.stack.drain(nargs),
            has_this,
            self.vm_config,
        );
        if !discard {
            self.stack
                .push(interpreter.eval_instructions(sheet, Mode::Fast)?);
        }
        Ok(())
    }

    pub(super) fn call_linkage(
        &mut self,
        linkage: __Linkage,
        nargs: u8,
        output_ty: EntityRoutePtr,
    ) -> __VMResult<()> {
        todo!()
    }
}
