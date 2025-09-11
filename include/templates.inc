// remove built-in include guard
#if defined _inc_templates
	#undef _inc_templates
#endif
// custom include-guard
#if defined _templates_included
	#endinput
#endif
#define _templates_included

#if !defined MAX_TEMPLATE_LENGTH
    #define MAX_TEMPLATE_LENGTH         (1024)
#endif

enum
{
    TYPE_STR = 1,
    TYPE_INT,
    TYPE_FLOAT
};

#define PAIR_STR(%0,%1) TYPE_STR, %0, %1
#define PAIR_INT(%0,%1) TYPE_INT, %0, %1
#define PAIR_FLOAT(%0,%1) TYPE_FLOAT, %0, _:%1

native Template:CreateTemplate(const template[]);
native RenderTemplate(Template:id, dest[], len, ...);

stock ReturnTemplate(Template:id, ...)
{    
    static buffer[MAX_TEMPLATE_LENGTH];
    #if defined _INC_y_va
        RenderTemplate(id, buffer, sizeof buffer, ___(1));
        return buffer;
    #else
        const static_args = 1;
        const args_diff = (3 - static_args) * 4;
        
        static args;
        args = numargs();

        if (args > static_args)
        {
            while (--args >= static_args)
            {
                #emit LCTRL     5
                #emit LOAD.alt  args
                #emit SHL.C.alt 2
                #emit ADD.C     12
                #emit ADD
                #emit LOAD.I
                #emit PUSH.pri
            }
            
            #emit PUSH.C        256
            #emit PUSH.C        buffer
            #emit PUSH.S        id
            #emit LOAD.S.pri    8
            #emit ADD.C         args_diff
            #emit PUSH.pri
            #emit SYSREQ.C      RenderTemplate
            #emit LCTRL         5
            #emit SCTRL         4
        }
        else
        {
            RenderTemplate(id, buffer, sizeof buffer);
        }
        return buffer;
    #endif
}

#if defined _PawnPlus_included
stock String:RenderTemplateStr(Template:id, ...)
{
    static buffer[MAX_TEMPLATE_LENGTH];
    #if defined _INC_y_va
        RenderTemplate(id, buffer, sizeof buffer, ___(1));
        return str_new(buffer);
    #else
        const static_args = 1;
        const args_diff = (3 - static_args) * 4;
        
        static args;
        args = numargs();

        if (args > static_args)
        {
            while (--args >= static_args)
            {
                #emit LCTRL     5
                #emit LOAD.alt  args
                #emit SHL.C.alt 2
                #emit ADD.C     12
                #emit ADD
                #emit LOAD.I
                #emit PUSH.pri
            }
            
            #emit PUSH.C        256
            #emit PUSH.C        buffer
            #emit PUSH.S        id
            #emit LOAD.S.pri    8
            #emit ADD.C         args_diff
            #emit PUSH.pri
            #emit SYSREQ.C      RenderTemplate
            #emit LCTRL         5
            #emit SCTRL         4
        }
        else
        {
            RenderTemplate(id, buffer, sizeof buffer);
        }
        return str_new(buffer);
    #endif
}
#endif